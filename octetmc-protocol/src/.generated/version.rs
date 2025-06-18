const _ : () = {
    const GENERATED : &semver::Version = &semver::Version {
        major : 1,
        minor : 21,
        patch : 6,
        pre   : semver::Prerelease::EMPTY,
        build : semver::BuildMetadata::EMPTY
    };
    if (LATEST_GAME_VERSION.major != GENERATED.major
        || LATEST_GAME_VERSION.minor != GENERATED.minor
        || LATEST_GAME_VERSION.patch != GENERATED.patch
    ) { panic!("mismatched generated output version"); }
};
