# Kotoba : Twitch chat tools


Aplication to interact with the twitch chat , connect and create diferents tools

## Goals
- Fully local
- Easy to connect

## Features

- easy log in from the twitch
- show the chat view 

## development features

- Cool design using React
- integrate a ux lirbaries
- backend writen in C#
- use sass and tailwind 4
- create test on dotnet and investigate if is posible do playwright tests
- or use c# to scrap with selenium ( to scrap the youtube chat in the future)

## dotnet

I use this proyect to practice dotnet and react so as well is a Tauri app, the backend will be developed on dotnet

to do this first we need to create a src folder for dotnet

`mkdir src-dotnet`

then inside the folder create a new lib

`dotnet new classlib --name kotoba.TauriPlugin`

## Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

Template created! To get started run:
  cd kotoba-chat-tools
  deno install
  deno task tauri android init

For Desktop development, run:
  deno task tauri dev

For Android development, run:
  deno task tauri android dev

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
