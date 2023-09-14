// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
import { lint } from '@node-rs/deno-lint'

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
	console.log('decorator sample is activated');

	function runLint(activeEditor: vscode.TextEditor) {
		const text = activeEditor.document.getText();
		const filename = activeEditor.document.fileName;
		console.log(filename)
		const result = lint(filename, text)
		console.log(result);
	}

	let timeout: NodeJS.Timer | undefined = undefined;
	function triggerLint(activeEditor: vscode.TextEditor, throttle = false) {
		if (timeout) {
			clearTimeout(timeout);
			timeout = undefined;
		}
		if (throttle) {
			timeout = setTimeout(()=>runLint(activeEditor), 500);
		} else {
			runLint(activeEditor);
		}
	}

	if (vscode.window.activeTextEditor) {
		triggerLint(vscode.window.activeTextEditor);
	}

	vscode.window.onDidChangeActiveTextEditor(editor => {
		if (editor) {
			triggerLint(editor);
		}
	}, null, context.subscriptions);

	vscode.workspace.onDidChangeTextDocument(event => {
		const activeEditor = vscode.window.activeTextEditor;
		if (!activeEditor) {
			return;
		}
		if (event.document !== activeEditor.document) {
			return;
		}
		if (
			!["javascript", "javascriptreact", "typescript", "typescriptreact"]
				.includes(activeEditor.document.languageId)
		) {
			return;
		}
		triggerLint(activeEditor, true);
	}, null, context.subscriptions);
}

// This method is called when your extension is deactivated
export function deactivate() {}
