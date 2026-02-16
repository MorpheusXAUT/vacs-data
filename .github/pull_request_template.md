# Dataset Changes

Thank you for your contribution to the `vacs` dataset.

> [!WARNING]
> Please make sure your dataset is in the correct `dataset/{FIR}/` directory, otherwise it will not be picked up by our validations and release process.

If you're contributing configuration for a new FIR, please add a small paragraph about your affiliation to the respective FIR to this PR description. We'll be glad to accept your contribution, but would like to ensure our dataset's origin is transparent.  
After the first PR was merged, we'll add you to the CODEOWNERS file for that FIR, so that future changes can be reviewed by you.

If you're contributing for an existing FIR, we'll wait for feedback from the current CODEOWNER(s) before merging.

_You may just delete these paragraphs from your PR description, they're just here to remind you of our contribution guidelines._

## Auto-Formatting

This repository uses [autofix.ci](https://autofix.ci) to automatically ensure consistent code formatting using our [prettier](https://prettier.io) configuration.
The `autofix-ci` bot will automatically push formatting fixes to your branch if needed.

> [!IMPORTANT]
> Please **enable "Allow edits by maintainers"** so that the bot can push commits to your PR branch.

Look for the **"Allow edits by maintainers"** checkbox when creating the PR (below the description box or in the right sidebar). Please ensure this checkbox is checked before submitting the PR.

The `autofix-ci` bot will automatically push formatting fixes to your branch if needed (commits will have the message `chore(dataset): format with prettier`).

<details>
<summary>If you prefer not to enable this setting:</summary>

Make sure your dataset changes are formatted properly.  
If your editor supports `prettier`, formatting should happen automatically.

**Malformed dataset files will automatically be rejected by CI.**

Alternatively, you can run our formatter locally:

1. Install [Node.js](https://nodejs.org/en/download)
2. Run `npm install` to install `prettier` and the required plugins
3. Run `npm run format` to format your dataset changes
</details>
