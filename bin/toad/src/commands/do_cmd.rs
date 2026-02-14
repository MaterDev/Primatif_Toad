use anyhow::Result;
use toad_core::BatchOperationReport;

pub fn log_audit(targets_count: usize, command: String, report: &BatchOperationReport) -> Result<()> {
    let entry = toad_ops::audit::AuditEntry {
        timestamp: chrono::Local::now().to_rfc3339(),
        command,
        target_count: targets_count,
        success_count: report.success_count,
        fail_count: report.fail_count,
        skip_count: report.skip_count,
        user: whoami::username().unwrap_or_else(|_| "unknown".to_string()),
    };
    toad_ops::audit::log_operation(entry)?;
    Ok(())
}
