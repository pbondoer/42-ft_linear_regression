# ft\_linear\_regression @ 42

![Screenshot](https://i.imgur.com/Tk729e9.png)

Rust implementation of the linear regression project.

## Compiling

You can compile the project with `cargo build`.

## Running
### Training program

```bash
cargo run --bin train
```

This will train with the dataset in `data/data.csv`.

This program will write it's results to `data/values`.

### Predicting program

```bash
cargo run --bin predict
```

This will predict a price based on the training done from the previous program.

### Visualizer

You can open `visualizer.html` in your favorite browser, drag-and-drop the
`data.csv` and `values` files, and preview the dataset.

## License

![GPL logo](https://www.gnu.org/graphics/gplv3-127x51.png "GNU General Public License")

`ft_linear_regression` is licensed under the [**GNU General Public License
3.0**](LICENSE).
