# SWON Playground - VS Code Web

This directory contains a TypeScript application that sets up a web build of VS Code in the browser for the SWON project. The initial implementation provides the boilerplate structure for running VS Code on the web, with future plans to support a WASI runtime for running a WASI LSP server.

## Project Structure

- `src/index.html` - HTML template for the web application
- `src/index.ts` - Main TypeScript entry point
- `webpack.config.js` - Webpack configuration for building the application
- `tsconfig.json` - TypeScript configuration

## Dependencies

The project uses the following dependencies:
- TypeScript for type-safe JavaScript development
- Webpack for bundling the application
- VS Code web-related packages:
  - `@vscode/codicons` - VS Code icons
  - `vscode-oniguruma` - Text mate grammar support
  - `vscode-textmate` - TextMate grammar parsing
  - `@vscode/vscode-languagedetection` - Language detection

## VS Code Web Configuration

The current implementation provides a minimal setup for VS Code web. The configuration includes:
- Basic editor container setup
- Loading of required VS Code web dependencies
- Placeholder for future VS Code web initialization

## Build Instructions

To build and run the project:

1. Install dependencies:
   ```
   npm install
   ```

2. Start the development server:
   ```
   npm start
   ```
   This will open the application in your default browser.

3. Build for production:
   ```
   npm run build
   ```
   This will create a production build in the `dist` directory.

## Future Enhancements

- Integration with SWON
- WASI runtime support for running a WASI LSP server
- Full VS Code web integration with custom extensions
