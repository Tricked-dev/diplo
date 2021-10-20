// use ::worker_sys::Response as EdgeResponse;


use worker::{worker_sys::Response as EdgeResponse, *};
mod github_struct;
mod utils;
use reqwest::header::USER_AGENT;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(||"unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // utils::set_panic_hook();
    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get("/installer.ps1", |_, _| {
            Response::ok(
                r##"#!/usr/bin/env pwsh
# Diplo installer - forked from https://deno.land/x/install/install.ps1

$ErrorActionPreference = 'Stop'

if ($args.Length -eq 1) {
  $Version = $args.Get(0)
}

$DiploInstall = $env:DENO_INSTALL
$BinDir = if ($DiploInstall) {
  "$DiploInstall\bin"
} else {
  "$Home\.diplo\bin"
}

$DiploZip = "$BinDir\diplo.zip"
$DiploExe = "$BinDir\diplo.exe"
$Target = 'x86_64-pc-windows-msvc'

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$DiploUri = "https://diplo.tricked.pro/download/86_64-windows.zip"

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest $DiploUri -OutFile $DiploZip -UseBasicParsing

if (Get-Command Expand-Archive -ErrorAction SilentlyContinue) {
  Expand-Archive $DiploZip -Destination $BinDir -Force
} else {
  if (Test-Path $DiploExe) {
    Remove-Item $DiploExe
  }
  Add-Type -AssemblyName System.IO.Compression.FileSystem
  [IO.Compression.ZipFile]::ExtractToDirectory($DiploZip, $BinDir)
}

Remove-Item $DiploZip

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}

Write-Output "Diplo was installed successfully to $DiploExe"
Write-Output "Run 'diplo --help' to get started""##,
            )
        })
        .get_async("/download/:platform", |_req, ctx| async move {
            if let Some(name) = ctx.param("platform") {
                let client = reqwest::Client::new();
                let result = client
                    .get("https://api.github.com/repos/tricked-dev/diplo/releases/latest")
                    .header(USER_AGENT, "cloudflare/worker")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let data: github_struct::GithubResponse = serde_json::from_str(&result)?;

                let download = match name {
                    name if name == "86_64-windows.zip" => {
                        let asset = data
                            .assets
                            .iter()
                            .find(|s| s.name.contains("windows"))
                            .unwrap();
                        &asset.browser_download_url
                    }

                    name if name == "86_64-macos.tar.xz" => {
                        let asset = data
                            .assets
                            .iter()
                            .find(|s| s.name.contains("macos"))
                            .unwrap();
                        &asset.browser_download_url
                    }

                    name if name == "aarch64-linux.tar.xz" => {
                        let asset = data
                            .assets
                            .iter()
                            .find(|s| s.name.contains("aarch64-linux"))
                            .unwrap();
                        &asset.browser_download_url
                    }
                    _ => {
                        let asset = data
                            .assets
                            .iter()
                            .find(|s| s.name.contains("86_64-linux"))
                            .unwrap();
                        &asset.browser_download_url
                    }
                };

                return match EdgeResponse::redirect(download) {
                    Ok(edge_response) => Ok(Response::from(edge_response)),
                    Err(err) => Err(Error::from(err)),
                };
            }

            Response::ok("Hello from Workers!")
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
