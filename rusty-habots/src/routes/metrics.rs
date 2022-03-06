use crate::data::UberRepository;
use crate::model::{Charts, Metric};
use log::{info, warn};
use rocket::http::{RawStr, Status};
use rocket::State;
use rocket_contrib::json::Json;
use std::time::{SystemTime};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

#[post("/users/<user_id>/habits/<habit_id>/metrics", data = "<json>")]
pub fn log_metric(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    user_id: &RawStr,
    habit_id: &RawStr,
    json: Json<Metric>,
) -> Result<Json<Metric>, Status> {
    info!("Submitted user {:?}", json);
    let mut metric = json.into_inner();
    metric.user = user_id.to_string();
    metric.habit = habit_id.to_string();
    metric.time = Some(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
    let result = repo.log_metric(&mut metric);
    if let Err(err) = result {
        warn!("Encountered error {:?}", err);
        return Err(Status::InternalServerError);
    }
    Ok(Json(metric))
}

#[get("/users/<_user_id>/habits/<habit_id>/metrics")]
pub fn get_metrics(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    _user_id: &RawStr,
    habit_id: &RawStr,
) -> Result<Json<Vec<Metric>>, Status> {
    info!("Find metrics for {:?}", habit_id);
    let result = repo.find_metrics(habit_id.to_string());
    match result {
        Some(metrics) => Ok(Json(metrics)),
        None => Err(Status::NotFound),
    }
}

const QUICKCHART: &str = "https://quickchart.io/chart";

pub fn get_charts(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    _user_id: &RawStr,
    habit_id: &RawStr,
) -> Result<Json<Charts>, Status> {
    let result = repo.find_metrics(habit_id.to_string());
    match result {
        Some(metrics) => Ok(Json(build_charts(&metrics))),
        None => Err(Status::NotFound),
    }
}

fn build_charts(metrics: &Vec<Metric>) -> Charts {
    return Charts {
        progress: "".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Chart {

    #[serde(rename = "type")]
    chart_type: String,

    #[serde(rename = "data")]
    data: Data,

    options: Options
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Data {

    #[serde(rename = "datasets")]
    datasets: Vec<Dataset>

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Dataset {

    #[serde(rename = "label")]
    label : String,

    #[serde(rename = "data")]
    data: Vec<u32>,

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Options {

    #[serde(rename = "rotation")]
    rotation: String,

    #[serde(rename = "circumference")]
    circumference: String,

    #[serde(rename = "cutoutPercentage")]
    cutout_percentage: u32,

    #[serde(rename = "plugins")]
    plugins: Plugins

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Plugins {

    #[serde(rename = "datalabels")]
    datalabels: Datalabels,

    #[serde(rename = "doughnutlabel", skip_serializing_if = "Option::is_none")]
    doughnutlabel: Option<DoughnutLabel>

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Datalabels {

    #[serde(rename = "display")]
    display: bool

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DoughnutLabel {

    #[serde(rename = "labels")]
    labels: Vec<Label>

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Label {

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(rename = "font")]
    font: Font

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Font {

    #[serde(rename = "size")]
    size: String

}

#[derive(PartialEq, Debug)]
struct Stats {

    success: u8

}

fn into_weekly_figures(metrics: &Vec<Metric>) -> Stats {
    Stats {
        success: 0
    }
}

fn build_progress_chart(metrics: &Vec<Metric>) -> Chart {
    Chart {
        chart_type: "doughnut".to_string(),
        data: Data {
            datasets: vec![Dataset {
                label: "foo".to_string(),
                data: vec![0, 100]
            }]
        },
        options: Options {
            rotation: "Math.PI".to_string(),
            circumference: "Math.PI".to_string(),
            cutout_percentage: 75,
            plugins: Plugins {
                datalabels: Datalabels {
                    display: false
                },
                doughnutlabel: Some(DoughnutLabel {
                    labels: vec![Label {
                        text: "\nHabit Success Rate".to_string(),
                        color: Some("#aaa".to_string()),
                        font: Font {
                            size: "25".to_string()
                        }
                    }, Label {
                        text: "\n0%".to_string(),
                        color: None,
                        font: Font {
                            size: "40".to_string()
                        }
                    }]
                })
            }
        }
    }
}

fn as_chart_uri(chart:Chart) -> Result<String, Status> {
    match serde_json::to_string(&chart) {
        Ok(json) => {
            let repl = json.replace("\"Math.PI\"", "Math.PI");
            let encoded = encode(repl.as_str()).to_string();
            Ok(format!("{}?c={}", QUICKCHART, encoded))
        },
        Err(err) => Err(Status::InternalServerError)
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_no_metrics() {
        let chart = build_progress_chart(&Vec::new());
        assert_eq!(Chart {
            chart_type: "doughnut".to_string(),
            data: Data {
                datasets: vec![Dataset {
                    label: "foo".to_string(),
                    data: vec![0, 100]
                }]
            },
            options: Options {
                rotation: "Math.PI".to_string(),
                circumference: "Math.PI".to_string(),
                cutout_percentage: 75,
                plugins: Plugins {
                    datalabels: Datalabels {
                        display: false
                    },
                    doughnutlabel: Some(DoughnutLabel {
                        labels: vec![Label {
                            text: "\nHabit Success Rate".to_string(),
                            color: Some("#aaa".to_string()),
                            font: Font {
                                size: "25".to_string()
                            }
                        }, Label {
                            text: "\n0%".to_string(),
                            color: None,
                            font: Font {
                                size: "40".to_string()
                            }
                        }]
                    })
                }
            }
        }, chart)
    }

    #[test]
    fn test_to_uri() {
        let uri = as_chart_uri(Chart {
            chart_type: "doughnut".to_string(),
            data: Data {
                datasets: vec![Dataset {
                    label: "foo".to_string(),
                    data: vec![0, 100]
                }]
            },
            options: Options {
                rotation: "Math.PI".to_string(),
                circumference: "Math.PI".to_string(),
                cutout_percentage: 75,
                plugins: Plugins {
                    datalabels: Datalabels {
                        display: false
                    },
                    doughnutlabel: Some(DoughnutLabel {
                        labels: vec![Label {
                            text: "\nHabit Success Rate".to_string(),
                            color: Some("#aaa".to_string()),
                            font: Font {
                                size: "25".to_string()
                            }
                        }, Label {
                            text: "\n0%".to_string(),
                            color: None,
                            font: Font {
                                size: "40".to_string()
                            }
                        }]
                    })
                }
            }
        });
        assert_eq!("https://quickchart.io/chart?c=%7B%22type%22%3A%22doughnut%22%2C%22data%22%3A%7B%22datasets%22%3A%5B%7B%22label%22%3A%22foo%22%2C%22data%22%3A%5B0%2C100%5D%7D%5D%7D%2C%22options%22%3A%7B%22rotation%22%3AMath.PI%2C%22circumference%22%3AMath.PI%2C%22cutoutPercentage%22%3A75%2C%22plugins%22%3A%7B%22datalabels%22%3A%7B%22display%22%3Afalse%7D%2C%22doughnutlabel%22%3A%7B%22labels%22%3A%5B%7B%22text%22%3A%22%5CnHabit%20Success%20Rate%22%2C%22color%22%3A%22%23aaa%22%2C%22font%22%3A%7B%22size%22%3A%2225%22%7D%7D%2C%7B%22text%22%3A%22%5Cn0%25%22%2C%22font%22%3A%7B%22size%22%3A%2240%22%7D%7D%5D%7D%7D%7D%7D", uri.unwrap());
    }

    #[test]
    fn test_none_at_all() {
        let stats = into_weekly_figures(&Vec::new());
        assert_eq!(Stats {
            success: 0
        }, stats);
    }

    #[test]
    fn test_none_this_week() {
        let metric = Metric {
            id: Some("1".to_string()),
            habit: "2".to_string(),
            user: "3".to_string(),
            time: Some(100)
        };
        let stats = into_weekly_figures(&Vec::new());
        assert_eq!(Stats {
            success: 0
        }, stats);
    }

    const DAY:u64 = 60 * 60 * 24;

    #[test]
    fn test_three_days() {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Need system time").as_secs();
        let metrics = vec![Metric {
            id: Some("1".to_string()),
            habit: "2".to_string(),
            user: "3".to_string(),
            time: Some(now - DAY)
        }, Metric {
            id: Some("1".to_string()),
            habit: "2".to_string(),
            user: "3".to_string(),
            time: Some(now -  DAY - DAY)
        }, Metric {
            id: Some("1".to_string()),
            habit: "2".to_string(),
            user: "3".to_string(),
            time: Some(now -  DAY - DAY - DAY)
        }];
        let stats = into_weekly_figures(&metrics);
        assert_eq!(Stats {
            success: 3
        }, stats);
    }

    #[test]
    fn test_multi_days() {

    }

}
