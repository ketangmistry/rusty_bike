resource "google_cloud_run_service" "rusty_bike" {
  name     = "rusty-bike"
  location = var.region

  template {
    spec {
      containers {
        image = "${var.region}-docker.pkg.dev/${var.project_id}/containers/rusty_bike:2b3649f"
        env {
          name = "ROCKET_PORT"
          value = 8000
        }
        env {
          name = "BIKES_YAML_FILE"
          value = "/app/bikes.yaml"
        }
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

resource "google_cloud_run_service_iam_binding" "rusty_bike" {
  location = google_cloud_run_service.rusty_bike.location
  service  = google_cloud_run_service.rusty_bike.name
  
  role     = "roles/run.invoker"

  members = [
    "allUsers"
  ]

}

