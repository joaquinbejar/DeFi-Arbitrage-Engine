#!/bin/bash

# DeFi Arbitrage Engine Database Backup Script
# This script creates automated backups of the TimescaleDB database

set -euo pipefail

# Configuration
DB_HOST="timescaledb"
DB_PORT="5432"
DB_NAME="arbitrage_db"
DB_USER="arbitrage_user"
BACKUP_DIR="/backups"
RETENTION_DAYS=30
DATE=$(date +"%Y%m%d_%H%M%S")
BACKUP_FILE="${BACKUP_DIR}/arbitrage_db_${DATE}.sql"
COMPRESSED_FILE="${BACKUP_FILE}.gz"

# Logging function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Error handling
error_exit() {
    log "ERROR: $1"
    exit 1
}

# Check if backup directory exists
if [ ! -d "$BACKUP_DIR" ]; then
    log "Creating backup directory: $BACKUP_DIR"
    mkdir -p "$BACKUP_DIR" || error_exit "Failed to create backup directory"
fi

# Wait for database to be ready
log "Waiting for database to be ready..."
until pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME"; do
    log "Database not ready, waiting..."
    sleep 5
done

log "Starting database backup..."

# Create database backup
pg_dump -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
    --verbose \
    --no-password \
    --format=custom \
    --compress=9 \
    --file="$BACKUP_FILE" || error_exit "Database backup failed"

# Compress the backup
log "Compressing backup file..."
gzip "$BACKUP_FILE" || error_exit "Backup compression failed"

# Verify backup integrity
log "Verifying backup integrity..."
if [ -f "$COMPRESSED_FILE" ]; then
    BACKUP_SIZE=$(stat -f%z "$COMPRESSED_FILE" 2>/dev/null || stat -c%s "$COMPRESSED_FILE" 2>/dev/null)
    if [ "$BACKUP_SIZE" -gt 1000 ]; then
        log "Backup created successfully: $COMPRESSED_FILE (${BACKUP_SIZE} bytes)"
    else
        error_exit "Backup file is too small, possible corruption"
    fi
else
    error_exit "Backup file not found after compression"
fi

# Clean up old backups
log "Cleaning up old backups (older than $RETENTION_DAYS days)..."
find "$BACKUP_DIR" -name "arbitrage_db_*.sql.gz" -type f -mtime +$RETENTION_DAYS -delete || log "Warning: Failed to clean up some old backups"

# Create backup metadata
METADATA_FILE="${BACKUP_DIR}/backup_metadata_${DATE}.json"
cat > "$METADATA_FILE" << EOF
{
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "database": "$DB_NAME",
    "host": "$DB_HOST",
    "backup_file": "$(basename "$COMPRESSED_FILE")",
    "file_size_bytes": $BACKUP_SIZE,
    "retention_days": $RETENTION_DAYS,
    "pg_version": "$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT version();" | head -1 | xargs)",
    "tables_count": $(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" | xargs),
    "backup_type": "full",
    "compression": "gzip"
}
EOF

# Optional: Upload to cloud storage (uncomment and configure as needed)
# upload_to_cloud() {
#     log "Uploading backup to cloud storage..."
#     # AWS S3 example:
#     # aws s3 cp "$COMPRESSED_FILE" "s3://your-backup-bucket/arbitrage-engine/$(basename "$COMPRESSED_FILE")"
#     # Google Cloud Storage example:
#     # gsutil cp "$COMPRESSED_FILE" "gs://your-backup-bucket/arbitrage-engine/$(basename "$COMPRESSED_FILE")"
#     # Azure Blob Storage example:
#     # az storage blob upload --file "$COMPRESSED_FILE" --name "arbitrage-engine/$(basename "$COMPRESSED_FILE")" --container-name backups
# }

# Uncomment to enable cloud upload
# upload_to_cloud

# Generate backup report
BACKUP_COUNT=$(find "$BACKUP_DIR" -name "arbitrage_db_*.sql.gz" -type f | wc -l)
TOTAL_SIZE=$(find "$BACKUP_DIR" -name "arbitrage_db_*.sql.gz" -type f -exec stat -f%z {} + 2>/dev/null | awk '{sum+=$1} END {print sum}' || \
             find "$BACKUP_DIR" -name "arbitrage_db_*.sql.gz" -type f -exec stat -c%s {} + 2>/dev/null | awk '{sum+=$1} END {print sum}')

log "Backup completed successfully!"
log "Current backup: $COMPRESSED_FILE"
log "Total backups: $BACKUP_COUNT"
log "Total backup size: ${TOTAL_SIZE} bytes"

# Optional: Send notification (uncomment and configure as needed)
# send_notification() {
#     local status="$1"
#     local message="$2"
#     
#     # Slack webhook example:
#     # curl -X POST -H 'Content-type: application/json' \
#     #     --data "{\"text\":\"Arbitrage Engine Backup $status: $message\"}" \
#     #     "$SLACK_WEBHOOK_URL"
#     
#     # Discord webhook example:
#     # curl -X POST -H 'Content-type: application/json' \
#     #     --data "{\"content\":\"Arbitrage Engine Backup $status: $message\"}" \
#     #     "$DISCORD_WEBHOOK_URL"
#     
#     # Email example (requires mailutils):
#     # echo "$message" | mail -s "Arbitrage Engine Backup $status" admin@example.com
# }

# send_notification "SUCCESS" "Database backup completed successfully. File: $(basename "$COMPRESSED_FILE"), Size: ${BACKUP_SIZE} bytes"

log "Backup process finished."
exit 0