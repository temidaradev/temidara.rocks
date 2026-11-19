# Homeserver deployment

GitHub Actions checks pull requests and pushes to `master`. After checks pass on
`master`, the runner joins Tailscale, syncs the source over SSH, and builds on
the homeserver. Each release contains the server binary and its matching
CSS, JavaScript, and WASM. The running service stays up during the build.
Activation switches the `current` symlink and restarts systemd. If the local
HTTP check fails, the script restores and restarts the previous release.
Expect a brief restart when a release goes live.

## GitHub and Tailscale

Open [the portfolio's Actions settings](https://github.com/temidaradev/temidara.rocks/settings/secrets/actions).
GitHub stores existing secrets write-only, including the ones in the Kopuz
repository. Create fresh credentials for this repository if you no longer
have the originals.

| Kind | Name | Where to get the value |
| --- | --- | --- |
| Variable | `DEPLOY_HOST` | The portfolio server's Tailscale hostname. For the same server as Kopuz, copy the value from the Kopuz repository's Actions variables. |
| Secret | `TS_OAUTH_CLIENT_ID` | Client ID from a Tailscale OAuth credential with **Auth keys → Write** for `tag:ci`. |
| Secret | `TS_OAUTH_SECRET` | Client secret shown when that Tailscale OAuth credential is created. |
| Secret | `DEPLOY_SSH_PRIVATE_KEY_B64` | Single-line Base64 encoding of the dedicated SSH private key below. |
| Secret | `DEPLOY_SSH_KNOWN_HOSTS` | The verified SSH host key entry for the exact hostname in `DEPLOY_HOST`. |

The existing Kopuz tailnet policy can be reused if it already allows `tag:ci`
to reach port 22 on the same homeserver. Create the OAuth credential under
**Trust credentials → Credential → OAuth**. See
[Tailscale's OAuth setup](https://tailscale.com/docs/features/oauth-clients#setting-up-an-oauth-client).

From a machine connected to the tailnet, create a dedicated deployment key
and authorize its public key:

```bash
deploy_host=YOUR_SERVER_TAILSCALE_NAME
ssh-keygen -t ed25519 -N '' -f ~/.ssh/temidara-rocks-ci -C temidara-rocks-ci
ssh-copy-id -i ~/.ssh/temidara-rocks-ci.pub "temidaradev@$deploy_host"
ssh-keyscan -t ed25519 "$deploy_host" > temidara-rocks-known-hosts
ssh-keygen -lf temidara-rocks-known-hosts
```

Compare the fingerprint to `ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub`
run on the homeserver. After they match, upload the settings from this
checkout with an authenticated GitHub CLI:

```bash
gh variable set DEPLOY_HOST --body "$deploy_host"
gh secret set TS_OAUTH_CLIENT_ID
gh secret set TS_OAUTH_SECRET
base64 < ~/.ssh/temidara-rocks-ci | tr -d '\n' | gh secret set DEPLOY_SSH_PRIVATE_KEY_B64
gh secret set DEPLOY_SSH_KNOWN_HOSTS < temidara-rocks-known-hosts
```

## Prepare the homeserver

The workflow uses `/home/temidaradev/Projects/temidara.rocks` on the Linux
homeserver. The public portfolio is at `https://temidara.dev/`. The unit
assumes its reverse proxy reaches the portfolio on `127.0.0.1:3006`.
Confirm the current proxy upstream and service settings before installing
the unit. The service needs `rsync`, `curl`, `flock`
(from util-linux), a C compiler, `pkg-config`, and OpenSSL development
headers to build the server.

```bash
rustup toolchain install 1.96.0 --profile minimal --target wasm32-unknown-unknown
cargo +1.96.0 install --version 0.3.6 --locked cargo-leptos
systemctl show temidara-rocks.service -p FragmentPath -p WorkingDirectory -p ExecStart
```

As `temidaradev`, seed the release layout with the currently running binary
and its corresponding generated site directory. Replace the two source
paths with the actual paths from the existing deployment:

```bash
deploy_root=/home/temidaradev/Projects/temidara.rocks
legacy="$deploy_root/releases/legacy"
install -d -m 0755 "$deploy_root/source" "$legacy/site"
install -d -m 0700 "$deploy_root/shared/data"
install -m 0755 /path/to/running/temidaradev-rust "$legacy/temidaradev-rust"
cp -a /path/to/running/site/. "$legacy/site/"
printf 'legacy\n' > "$legacy/REVISION"
ln -s releases/legacy "$deploy_root/current"
```

Save a copy of the existing systemd unit before cutover. During cutover,
stop `temidara-rocks.service` before copying the live guestbook. Preserve the
current application variables in `$deploy_root/shared/portfolio.env`
(mode `0600`) using systemd's `KEY=value` format. This includes the SMTP,
guestbook moderation, ListenBrainz, and analytics settings in use today.
Copy the existing guestbook and cover cache from their configured paths
into `$deploy_root/shared/data/guestbook.json` and
`$deploy_root/shared/data/covers/`. The unit sets these two paths explicitly
so they survive release changes.

Install the new unit from this repository and start the seeded release:

```bash
sudo install -m 0644 systemd/temidara-rocks.service /etc/systemd/system/temidara-rocks.service
sudo systemctl daemon-reload
sudo systemctl enable --now temidara-rocks.service
curl -fsS http://127.0.0.1:3006/ >/dev/null
```

If that check fails, restore the saved unit, reload systemd, and restart
the service using the old deployment. Reinstall the unit and reload systemd
whenever the repository's unit changes.

Use `sudo visudo -f /etc/sudoers.d/temidara-rocks-deploy` to allow CI to
restart this specific service. Use the path from `command -v systemctl`
if it differs:

```sudoers
temidaradev ALL=(root) NOPASSWD: /usr/bin/systemctl restart temidara-rocks.service
```

Run **CI and deploy** from the repository's Actions tab on `master` once the
workflow and settings are in place. To verify activation on the homeserver,
check the release revision and public site:

```bash
cat /home/temidaradev/Projects/temidara.rocks/current/REVISION
curl -fsS https://temidara.dev/ >/dev/null
```
