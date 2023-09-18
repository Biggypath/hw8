
use csv::{ReaderBuilder, Writer, Trim};
use std::io::Read;
use std::fs::File;
fn main() {
   html_polar();
}
#[derive(Clone)]
struct Point { 
    x: f64,
    y: f64
}
#[derive(Clone)]
struct PolarPoint {
    r: f64,
    t: f64
}

fn to_polar(pt_list: Vec<Point>) -> Vec<PolarPoint> {
    let mut result = Vec::new();
    for cor in pt_list{
        let r = (cor.x.powi(2) + cor.y.powi(2)).sqrt();
        let t = (cor.y.atan2(cor.x)).to_degrees();
        result.push(PolarPoint { r: r, t: t })
    }
    result
}

fn to_cartesian(pt_list: Vec<PolarPoint>) -> Vec<Point> {
    let mut result = Vec::new();
    for cor in pt_list {
        let x = cor.r * cor.t.to_radians().cos();
        let y = cor.r * cor.t.to_radians().sin(); 
        result.push(Point { x: x, y: y}) 
    }
    result
}
//------------------------------------------------------------------------------------------------
fn load_point_car<R: Read>(rdr: R) -> Vec<Point> {
    let mut reader
        = ReaderBuilder::new() 
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All) 
        .from_reader(rdr);

        let mut out_list = vec![];
        for record in reader.records() {
            if let Ok(rec) = record {
            let x: f64 = rec[0].parse().unwrap(); 
            let y: f64 = rec[1].parse().unwrap(); 
            out_list.push( Point { x: x, y: y});
            } 
        }
    out_list 

}

fn load_point_polar<R: Read>(rdr: R) -> Vec<PolarPoint> {
    let mut reader
        = ReaderBuilder::new() 
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All) 
        .from_reader(rdr);

        let mut out_list = vec![];
        for record in reader.records() {
            if let Ok(rec) = record {
            let r: f64 = rec[0].parse().unwrap(); 
            let t: f64 = rec[1].parse().unwrap(); 
            out_list.push( PolarPoint { r: r, t: t});
            } 
        }
    out_list 

} 

fn save_points_car<W: std::io::Write>(writer: W, pt_list: Vec<Point>) {
    let mut wtr = Writer::from_writer(writer);
    for pt in pt_list {
        wtr.write_record(&[pt.x.to_string(), pt.y.to_string()])
            .unwrap();
    }
    wtr.flush().unwrap();
}

fn save_points_polar<W: std::io::Write>(writer: W, pt_list: Vec<PolarPoint>) {
    let mut wtr = Writer::from_writer(writer);
    for pt in pt_list {
        wtr.write_record(&[pt.r.to_string(), pt.t.to_string()])
            .unwrap();
    }
    wtr.flush().unwrap();
}

//2.1
#[allow(dead_code)]
fn read_car() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 {
        println!("-")
    }
    if args.len() < 3{
        return;
    } 
    
    let input_name: String = args[1].parse().unwrap();
    let output_name: String = args[2].parse().unwrap();
    let point = load_point_car(File::open(input_name).unwrap());
    let polar = to_polar(point);
    let result =  save_points_polar(File::create(output_name).unwrap(), polar);
    result
} 

//2.2
#[allow(dead_code)]
fn read_polar() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 {
        println!("-")
    }
    if args.len() < 3{
        return;
    } 
    
    let input_name: String = args[1].parse().unwrap();
    let output_name: String = args[2].parse().unwrap();
    let point = load_point_polar(File::open(input_name).unwrap());
    let cartesian = to_cartesian(point);
    let result =  save_points_car(File::create(output_name).unwrap(), cartesian);
    result
} 

//------------------------------------------------------------------------------------------------
//3.1
#[allow(dead_code)]
fn html_car() {
    let args: Vec<_> = std::env::args().collect();
    let input_name: String = args[1].parse().unwrap();
    let point = load_point_car(File::open(input_name).unwrap());
    let polar = to_polar(point.clone());
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>Cartesian to Polar</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 50%;
                border-collapse: collapse; 
                }
            </style>
            <h1>Cartesian to Polar</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>Cartesian</th>
                        <th>Polar</th>
                    </tr>
                </thead>
                <tbody>"
    );
    for (cor_cartesian, cor_polar) in point.iter().zip(polar.iter()) {
        table.push_str("<tr>");
        table.push_str(&format!("<td>({:.1}, {:.1})</td>", cor_cartesian.x, cor_cartesian.y));
        table.push_str(&format!("<td>({:.1}, {:.1}°)</td>", cor_polar.r, cor_polar.t));
        table.push_str("</tr>");
    }
    table.push_str("</tbody></table></body></html>");
    println!("{}", table)
    
}
//3.2
#[allow(dead_code)]
fn html_polar() {
    let args: Vec<_> = std::env::args().collect();
    let input_name: String = args[1].parse().unwrap();
    let point = load_point_polar(File::open(input_name).unwrap());
    let cartesian = to_cartesian(point.clone());
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>Polar to Cartesian</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 50%;
                border-collapse: collapse; 
                }
            </style>
            <h1>Polar to Cartesian</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>Polar</th>
                        <th>cartesian</th>
                    </tr>
                </thead>
                <tbody>"
    );
    for (cor_polar, cor_cartesian) in point.iter().zip(cartesian.iter()) {
        table.push_str("<tr>");
        table.push_str(&format!("<td>({:.1}, {:.1})</td>", cor_polar.r, cor_polar.t));
        table.push_str(&format!("<td>({:.1}, {:.1}°)</td>", cor_cartesian.x, cor_cartesian.y));
        table.push_str("</tr>");
    }
    table.push_str("</tbody></table></body></html>");
    println!("{}", table)
    
}