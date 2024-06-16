use csv::ReaderBuilder;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Record {
    x: f64,
    y: f64,
}

fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }

    Ok(records)
}

fn create_chart(records: Vec<Record>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("CSV Data Visualization", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..10f64, 0f64..10f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        records.iter().map(|record| {
            Circle::new((record.x, record.y), 5, RED.filled())
        }),
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let records = read_csv("data.csv")?;
    create_chart(records, "output.png")?;
    Ok(())
}
