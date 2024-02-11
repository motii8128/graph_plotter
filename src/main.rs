use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::*;

// USER SETTINGS
const MIN_X:f64 = -100.0;
const MAX_X:f64 = 100.0;
const DELTA:f64 = 0.10;

const Y_SIZE:f64 = 100.0;

// write the function that you want to see.
fn func(x:f64)->f64
{
    x.powi(2)
}

// USER SETTINGS


fn main()
{
    let func_result = plotter(func);
    let horizontal = plotter(horizontal_f);
    let vertical = ano_plotter(horizontal_f);
    let s0 = Plot::new(func_result).point_style(
        PointStyle::new()
            .colour("#ff0000")
            .size(1.0),
    );
    let h_s = Plot::new(horizontal).point_style(
        PointStyle::new()
            .colour("#000000")
            .size(1.0)
    );
    let v_s = Plot::new(vertical).point_style(
        PointStyle::new()
            .colour("#000000")
            .size(1.0)
    );
    let v = ContinuousView::new()
        .add(s0)
        .add(h_s)
        .add(v_s)
        .x_range(MIN_X as f64, MAX_X as f64)
        .y_range(-1.0 * Y_SIZE, Y_SIZE)
        .x_label("x")
        .y_label("y");
    Page::single(&v).save("./result.svg").unwrap();
}

fn horizontal_f(x:f64)->f64
{
    x * 0.0
}

fn plotter<F:Fn(f64)->f64>(f:F)->Vec<(f64, f64)>
{
    let mut result = vec![(0.0, 0.0)];

    let mut start = MIN_X;

    while start < MAX_X{
        result.push((start, f(start)));

        start += DELTA;
    }

    result
}

fn ano_plotter<F:Fn(f64)->f64>(f:F)->Vec<(f64, f64)>
{
    let mut result = vec![(0.0, 0.0)];

    let mut start = -1.0 * Y_SIZE;

    while start < Y_SIZE{
        result.push((f(start), start));

        start += DELTA;
    }

    result
}
