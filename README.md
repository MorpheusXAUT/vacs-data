# vacs Data Repository

This is the central data repository for [**vacs**](https://github.com/vacs-project/vacs), the **VATSIM ATC Communication System**. It contains the configuration data for Stations, Positions, and Profiles used by vacs to manage diverse ATC setups across the VATSIM network.

By separating configuration from the core application code, we allow FIRs to manage and update their own data independently and define their controllers' experience using vacs.

## Documentation

Comprehensive documentation on how to understand and contribute to this dataset is available in the **[Dataset Documentation](docs/dataset/README.md)**.

This includes guides for:

- [Stations](docs/dataset/stations.md)
- [Positions](docs/dataset/positions.md)
- [Profiles](docs/dataset/profiles.md)

## Tools

Because the dataset is large and complex, we provide a set of command-line tools to help maintain it. These tools can be used to validate the dataset against the schema and import data from other formats.

See the **[Tools Documentation](tools/README.md)** for more information.

## Contributing

We welcome contributions from FIR staff and community members! Please refer to the [Dataset Documentation](docs/dataset/README.md) for detailed instructions on the file formats and directory structure.

**Already a maintainer?** Create a branch directly in this repository, make your changes, and open a PR. Your fellow CODEOWNERS can review and approve it.

**Adding a new FIR?** [Fork the repository](https://github.com/vacs-project/vacs-data/fork), create your dataset, and open a PR. After your first contribution is accepted, we'll add you and your team as maintainers so you can work directly from the main repo going forward.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full contribution guidelines.

## License

This repository contains both a dataset and tools to manage it. Due to the different nature of the contents, multiple licenses apply.

### Dataset license

The dataset content of this repository is [licensed](LICENSE) under the **[Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0)](https://creativecommons.org/licenses/by-nc-sa/4.0/)** license.

**This means you are free to:**

- **Share** - copy and redistribute the material in any medium or format.
- **Adapt** - remix, transform, and build upon the material.

**Under the following validation terms:**

- **Attribution** - You must give appropriate credit, provide a link to the license, and indicate if changes were made.
- **NonCommercial** - You may not use the material for commercial purposes.
- **ShareAlike** - If you remix, transform, or build upon the material, you must distribute your contributions under the same license as the original.

### Tools license

The tools in this repository (`tools/`) are licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.
