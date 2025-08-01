const _ : () = {
    const GENERATED : &semver::Version = &semver::Version {
        major : 1,
        minor : 21,
        patch : 8,
        pre   : semver::Prerelease::EMPTY,
        build : semver::BuildMetadata::EMPTY
    };
    if (GAME_VERSION.major != GENERATED.major
        || GAME_VERSION.minor != GENERATED.minor
        || GAME_VERSION.patch != GENERATED.patch
    ) { panic!("mismatched generated output version"); }
};
