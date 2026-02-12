## Contributing to the Dataset

Thank you for your interest in contributing to the `vacs` dataset!

### Directory Structure

All dataset contributions must be placed in the correct directory:

```
dataset/{FIR}/
```

Where `{FIR}` is the two/four letter Flight Information Region code (e.g., `LO` for Austria, `EDMM` for Munich, etc.).

> [!IMPORTANT] Files outside the correct directory structure will not be picked up by our validation and release process.

### Auto-Formatting

This repository uses [autofix.ci](https://autofix.ci) with [Prettier](https://prettier.io) to ensure consistent code formatting.

When you create a pull request, please **enable "Allow edits by maintainers"**. This allows the autofix.ci bot to automatically push formatting fixes to your PR branch.

#### Setting up local formatting (optional)

If you prefer to format files locally before pushing:

```bash
# Install dependencies (one-time)
npm install

# Format all dataset files
npm run format

# Check formatting without modifying
npm run format:check
```

If your editor supports Prettier, formatting can happen automatically on save. See our editor configuration in `.vscode/settings.json` for VS Code setup.

### New FIR Contributions

If you're contributing configuration for a **new FIR**:

1. Add your dataset files to `dataset/{FIR}/`
2. In your pull request description, include a brief paragraph about your affiliation to the FIR
3. After your first PR is merged, we'll add you to the `CODEOWNERS` file for that FIR
4. Future changes to your FIR will be routed to you for review

### Existing FIR Contributions

If you're contributing to an **existing FIR**:

- Your PR will be reviewed by the current CODEOWNER(s) for that FIR
- We'll wait for their feedback before merging

### Validation

All contributions are automatically validated for:

- Valid JSON/TOML syntax
- Proper formatting (via Prettier)
- Schema compliance (structure, required fields, data types)

If validation fails, you'll receive feedback in your pull request with specific error messages.

---

## License for dataset

Any contribution intentionally submitted for inclusion in the dataset provided by the `vacs-data` project by you shall be licensed under the Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0) license.

In short: by contributing, you agree that your contributions may be used under the CC BY-NC-SA 4.0 license, the same way as the existing dataset.

By submitting a contribution, you represent that you have the right to do so (e.g., you are the author or have permission from the rights holder) and that you are granting the project and its users a license to your contribution under the same terms.

## License for tools

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the tools provided by the `vacs-data` project by you, as defined in the Apache-2.0 license, shall be dual-licensed under the MIT license and the Apache License, Version 2.0, at your option, without any additional terms or conditions.

In short: by contributing, you agree that your contributions may be used under either the MIT or the Apache-2.0 license, the same way as the existing code.

By submitting a contribution, you represent that you have the right to do so (e.g., your employer allows you to contribute under these terms) and that you are granting the project and its users a license to your contribution under the same terms.
