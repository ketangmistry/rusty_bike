resource "google_cloud_run_service" "rusty_bike" {
  name     = "rusty-bike"
  location = var.region

  template {
    spec {
      containers {
        image = "${var.region}-docker.pkg.dev/${var.project_id}/containers/rusty_bike:8433b46"
        ports {
          container_port = 8000
        }
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }

}

