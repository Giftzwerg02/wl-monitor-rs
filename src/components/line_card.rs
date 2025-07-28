use maud::{Markup, html};

use crate::api::Line;

pub fn line_card(line: &Line) -> Markup {
    html! {
        div class="card" {
            div class="card-body" {
                div class="card-title" { (line.name) }
                div class="card-subtitle" { (line.towards) }
                div class="card-text" {
                    p { "Abfahrten" }
                    table {
                        @for departure in &line.departures.departure {
                            tr {
                                td { (line.towards.trim()) }
                                td {
                                    @if departure.vehicle.folding_ramp { "_" }
                                    @else if departure.vehicle.barrier_free { "." }
                                    @else if line.barrier_free { "." }
                                }
                                td { (departure.departure_time.countdown) }
                            }
                        }
                    }
                }
            }
        }
    }
}
