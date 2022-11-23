output "run_status" {
  value       = google_cloud_run_service.rusty_bike.status
  description = "GCP Cloud Run Status"
}
