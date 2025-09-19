#!/bin/bash

# DeFi Arbitrage Engine Database Restore Script
# TimescaleDB restore script

set -euo pipefail

# Configuration
DB_HOST="timescaledb"
DB_PORT="5432"
DB_NAME="arbitrage_db"
DB_USER="arbitrage_user"
BACKUP_DIR="/backups"
TEMP_DIR="/tmp"

# Logging function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Error handling
error_exit() {
    log "ERROR: $1"
    exit 1
}

# Usage function
usage() {
    echo "Usage: $0 [OPTIONS] <backup_file>"
    echo "Options:"
    echo "  -h, --help              Show this help message"
    echo "  -f, --force             Force restore without confirmation"
    echo "  -c, --clean             Drop existing database before restore"
    echo "  -l, --list              List available backup files"
    echo "  -v, --verify            Verify backup file integrity only"
    echo "  --host HOST             Database host (default: $DB_HOST)"
    echo "  --port PORT             Database port (default: $DB_PORT)"
    echo "  --database DB           Database name (default: $DB_NAME)"
    echo "  --user USER             Database user (default: $DB_USER)"
    echo ""
    echo "Examples:"
    echo "  $0 arbitrage_db_20240101_120000.sql.gz"
    echo "  $0 --clean --force /backups/arbitrage_db_20240101_120000.sql.gz"
    echo "  $0 --list"
    echo "  $0 --verify arbitrage_db_20240101_120000.sql.gz"
}

# List available backups
list_backups() {
    log "Available backup files in $BACKUP_DIR:"
    if [ -d "$BACKUP_DIR" ]; then
        find "$BACKUP_DIR" -name "arbitrage_db_*.sql.gz" -type f -printf "%T@ %Tc %p\n" 2>/dev/null | \
            sort -nr | \
            awk '{print $2" "$3" "$4" "$5" "$6" - "$7}' || \
        find "$BACKUP_DIR" -name "arbitrage_db_*.sql.gz" -type f -exec ls -la {} \;
    else
        log "Backup directory does not exist: $BACKUP_DIR"
        exit 1
    fi
}

# Verify backup file integrity
verify_backup() {
    local backup_file="$1"
    
    log "Verifying backup file: $backup_file"
    
    # Check if file exists
    if [ ! -f "$backup_file" ]; then
        error_exit "Backup file not found: $backup_file"
    fi
    
    # Check file size
    local file_size
    file_size=$(stat -f%z "$backup_file" 2>/dev/null || stat -c%s "$backup_file" 2>/dev/null)
    if [ "$file_size" -lt 1000 ]; then
        error_exit "Backup file is too small (${file_size} bytes), possible corruption"
    fi
    
    # Test gzip integrity
    if [[ "$backup_file" == *.gz ]]; then
        log "Testing gzip integrity..."
        if ! gzip -t "$backup_file"; then
            error_exit "Backup file is corrupted (gzip test failed)"
        fi
    fi
    
    # Test PostgreSQL dump format
    local temp_file="${TEMP_DIR}/test_restore_$$"
    if [[ "$backup_file" == *.gz ]]; then
        zcat "$backup_file" > "$temp_file"
    else
        cp "$backup_file" "$temp_file"
    fi
    
    # Check if it's a valid PostgreSQL dump
    if ! head -n 10 "$temp_file" | grep -q "PostgreSQL database dump"; then
        rm -f "$temp_file"
        error_exit "File does not appear to be a valid PostgreSQL dump"
    fi
    
    rm -f "$temp_file"
    log "Backup file verification passed: $backup_file (${file_size} bytes)"
}

# Parse command line arguments
FORCE=false
CLEAN=false
LIST_ONLY=false
VERIFY_ONLY=false
BACKUP_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            exit 0
            ;;
        -f|--force)
            FORCE=true
            shift
            ;;
        -c|--clean)
            CLEAN=true
            shift
            ;;
        -l|--list)
            LIST_ONLY=true
            shift
            ;;
        -v|--verify)
            VERIFY_ONLY=true
            shift
            ;;
        --host)
            DB_HOST="$2"
            shift 2
            ;;
        --port)
            DB_PORT="$2"
            shift 2
            ;;
        --database)
            DB_NAME="$2"
            shift 2
            ;;
        --user)
            DB_USER="$2"
            shift 2
            ;;
        -*)
            error_exit "Unknown option: $1"
            ;;
        *)
            if [ -z "$BACKUP_FILE" ]; then
                BACKUP_FILE="$1"
            else
                error_exit "Multiple backup files specified"
            fi
            shift
            ;;
    esac
done

# Handle list-only mode
if [ "$LIST_ONLY" = true ]; then
    list_backups
    exit 0
fi

# Validate backup file argument
if [ -z "$BACKUP_FILE" ]; then
    error_exit "No backup file specified. Use --help for usage information."
fi

# Convert relative path to absolute if needed
if [[ "$BACKUP_FILE" != /* ]]; then
    if [ -f "${BACKUP_DIR}/${BACKUP_FILE}" ]; then
        BACKUP_FILE="${BACKUP_DIR}/${BACKUP_FILE}"
    elif [ -f "$BACKUP_FILE" ]; then
        BACKUP_FILE="$(realpath "$BACKUP_FILE")"
    else
        error_exit "Backup file not found: $BACKUP_FILE"
    fi
fi

# Verify backup file
verify_backup "$BACKUP_FILE"

# Handle verify-only mode
if [ "$VERIFY_ONLY" = true ]; then
    log "Backup file verification completed successfully."
    exit 0
fi

# Wait for database to be ready
log "Waiting for database to be ready..."
until pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER"; do
    log "Database not ready, waiting..."
    sleep 5
done

# Confirmation prompt (unless forced)
if [ "$FORCE" != true ]; then
    echo
    log "WARNING: This will restore the database '$DB_NAME' from backup."
    if [ "$CLEAN" = true ]; then
        log "WARNING: The --clean option will DROP the existing database first!"
    fi
    log "Backup file: $BACKUP_FILE"
    log "Database: $DB_USER@$DB_HOST:$DB_PORT/$DB_NAME"
    echo
    read -p "Are you sure you want to continue? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        log "Restore cancelled by user."
        exit 0
    fi
fi

# Create temporary directory for extraction
TEMP_RESTORE_DIR="${TEMP_DIR}/restore_$$"
mkdir -p "$TEMP_RESTORE_DIR"

# Cleanup function
cleanup() {
    log "Cleaning up temporary files..."
    rm -rf "$TEMP_RESTORE_DIR"
}
trap cleanup EXIT

# Extract backup file if compressed
RESTORE_FILE="$BACKUP_FILE"
if [[ "$BACKUP_FILE" == *.gz ]]; then
    log "Extracting compressed backup..."
    RESTORE_FILE="${TEMP_RESTORE_DIR}/$(basename "${BACKUP_FILE%.gz}")"
    gunzip -c "$BACKUP_FILE" > "$RESTORE_FILE" || error_exit "Failed to extract backup file"
fi

# Drop database if clean option is specified
if [ "$CLEAN" = true ]; then
    log "Dropping existing database..."
    
    # Terminate existing connections
    psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d postgres -c \
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$DB_NAME' AND pid <> pg_backend_pid();" || \
        log "Warning: Could not terminate existing connections"
    
    # Drop database
    dropdb -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" "$DB_NAME" || \
        log "Warning: Database may not exist or could not be dropped"
    
    # Create new database
    log "Creating new database..."
    createdb -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" "$DB_NAME" || \
        error_exit "Failed to create new database"
fi

# Restore database
log "Starting database restore..."
log "This may take several minutes depending on the backup size..."

start_time=$(date +%s)

# Perform the restore
if pg_restore -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
    --verbose \
    --no-password \
    --clean \
    --if-exists \
    --no-owner \
    --no-privileges \
    "$RESTORE_FILE"; then
    
    end_time=$(date +%s)
    duration=$((end_time - start_time))
    
    log "Database restore completed successfully in ${duration} seconds!"
    
    # Verify restore by checking table count
    table_count=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c \
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" | xargs)
    
    log "Restored database contains $table_count tables"
    
    # Update statistics
    log "Updating database statistics..."
    psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "ANALYZE;" || \
        log "Warning: Could not update statistics"
    
    echo "TimescaleDB restore completed successfully"
    
else
    echo "TimescaleDB restore failed"
fi

exit 0