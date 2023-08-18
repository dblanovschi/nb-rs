# 0.18.0

Released: 2023-08-18

- Added new categories: `handshake`, `lurk`, `peck` and `yawn`.

# 0.17.0

Released: 2023-04-24

- Implemented a proper `Client`.
- Added rate limiting for the search endpoint.
- Removed the unused `build-dependencies` section from `Cargo.toml`,
  which was previously used for the `local` feature. 

# 0.16.0

Released: 2023-04-16

- Due to the change from sequential IDs to UUIDs, the `local`
  feature no longer makes sense, and has been removed.