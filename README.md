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


## Why?

  - React + .Net stack is very popular
  - I have experience in both but I need a way to demostrate. this is the bad part of work in a private corpo workplace.
  - I don't want to create an API do complicated things to the user execute it in their machine or upload the api to a server.
  - all if paackaged in a nice and simple executable.
  - thanks Tauri, is multiplatform!!!!

## important documentation

  The video I take as inspiration to start this.

  https://www.youtube.com/watch?v=CDEO9qGRw20

# WIndows media control 

https://stackoverflow.com/questions/65011660/how-can-i-get-the-title-of-the-currently-playing-media-in-windows-10-with-python

https://microsoft.github.io/windows-docs-rs/doc/windows/Media/Control/struct.GlobalSystemMediaTransportControlsSessionManager.html#method.GetCurrentSession

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
