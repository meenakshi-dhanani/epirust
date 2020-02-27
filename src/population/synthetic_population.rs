use ndarray::{Array2, Axis, Array1};

pub type Matrix = Array2<f64>;

pub fn ipfn(margins: &Array1<Array1<f64>>) -> Matrix {
    let row_count = margins[0].len();
    let column_count = margins[1].len();
    let seed_matrix: Matrix = Array2::ones((row_count, column_count));
    //TODO: repeat the adjustment until difference between resultant matrices is within acceptable threshold - Jayanta
    let m1 = adjustment(&seed_matrix, &margins);
    let m2 = adjustment(&m1, &margins);

    m2
}

fn adjustment(current_matrix: &Matrix, margins: &Array1<Array1<f64>>) -> Matrix {
    let totals = compute_totals(&current_matrix);
    let row_adjusted_matrix =
        adjust(&current_matrix, &margins, &totals, true);
    let new_totals = compute_totals(&row_adjusted_matrix);
    let column_adjusted_matrix =
        adjust(&row_adjusted_matrix, &margins, &new_totals, false);
    column_adjusted_matrix
}

fn adjust(current_matrix: &Matrix, margins: &Array1<Array1<f64>>, totals: &(Array1<f64>, Array1<f64>), is_row_adjustment: bool) -> Matrix {
    if is_row_adjustment == true {
        let mut row_adjusted_matrix: Matrix = Matrix::zeros(current_matrix.dim());
        for ((x, y), value) in current_matrix.indexed_iter() {
            let row_total = &totals.0;
            row_adjusted_matrix[[x, y]] = value *
                margins.get(0).unwrap()[x] / row_total.get(x).unwrap();
        }
        return row_adjusted_matrix;
    }

    let mut column_adjusted_matrix: Matrix = Matrix::zeros(current_matrix.dim());
    for ((x, y), value) in current_matrix.indexed_iter() {
        let column_total = &totals.1;
        column_adjusted_matrix[[x, y]] = value *
            margins.get(1).unwrap()[y] / column_total.get(y).unwrap();
    }

    column_adjusted_matrix
}

fn column_adjust(current: &mut Matrix, last_iteration: &Matrix, row_count: usize, column_count: usize) {
    for ((x, y), _old_value) in last_iteration.indexed_iter() {
        if x != row_count && y != column_count {
            current[[x, y]] =
                (last_iteration[[x, column_count]] *
                    last_iteration[[row_count, y]]) / current[[row_count, y]];
        } else {
            current[[x, y]] = last_iteration[[x, y]];
        }
    }
}

fn compute_totals(matrix: &Matrix) -> (Array1<f64>, Array1<f64>) {
    return (matrix.sum_axis(Axis(1)), matrix.sum_axis(Axis(0)));
}

#[cfg(test)]
mod tests {
    use ndarray::arr1;
    use crate::population::synthetic_population::ipfn;

    #[test]
    fn should_generate_population_for_square_matrix() {
        let row_margin = arr1(&[5.0, 15.0, 8.0]);
        let column_margin = arr1(&[11.0, 8.0, 9.0]);

        let margins = arr1(&[row_margin, column_margin]);

        let citizen_distribution = ipfn(&margins);
        assert_eq!(citizen_distribution[[0, 0]], 1.9642857142857146);
    }

    #[test]
    fn should_generate_population() {
        let row_margin = arr1(&[45.0, 85.0, 45.0]);
        let column_margin = arr1(&[68.0, 54.0, 53.0]);

        let margins = arr1(&[row_margin, column_margin]);
        let citizen_distribution = ipfn(&margins);

        assert_eq!(citizen_distribution[[0, 0]], 17.485714285714288);
    }
}