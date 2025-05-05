/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import { workspace, ExtensionContext, commands, window } from 'vscode';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: ExtensionContext) {

	// Register the commands
	context.subscriptions.push(
		commands.registerCommand('swon-ls.start', startLanguageServer),
		commands.registerCommand('swon-ls.stop', stopLanguageServer),
		commands.registerCommand('swon-ls.restart', restartLanguageServer)
	);

	// Start the language server by default
	startLanguageServer();

	async function startLanguageServer() {
		if (client) {
			window.showInformationMessage('Swon language server is already running.');
			return;
		}

		// If the extension is launched in debug mode then the debug server options are used
		// Otherwise the run options are used
		const serverOptions: ServerOptions = {
			run: {
				command: "swon-ls",
				transport: TransportKind.stdio
			},
			debug: {
				command: "swon-ls",
				transport: TransportKind.stdio
			}
		};

		// Options to control the language client
		const clientOptions: LanguageClientOptions = {
			// Register the server for swon documents
			documentSelector: [{ scheme: 'file', language: 'swon' }],
			synchronize: {
				// Notify the server about file changes to '.clientrc files contained in the workspace
				fileEvents: workspace.createFileSystemWatcher('**/*.swon')
			}
		};

		// Create the language client and start the client.
		client = new LanguageClient(
			'swon-ls',
			'Swon Language Server',
			serverOptions,
			clientOptions
		);

		// Start the client. This will also launch the server
		await client.start();
		window.showInformationMessage('Swon language server started.');
	}

	async function stopLanguageServer() {
		if (!client) {
			window.showInformationMessage('Swon language server is not running.');
			return;
		}

		await client.stop();
		client = undefined;
		window.showInformationMessage('Swon language server stopped.');
	}

	async function restartLanguageServer() {
		window.showInformationMessage('Restarting Swon language server...');
		await stopLanguageServer();
		await startLanguageServer();
	}
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
