#!/bin/bash

# https://gist.github.com/kwmiebach/e42dc4a43d5a2a0f2c3fdc41620747ab
get_toml_value() {
    # Takes three parameters:
    # - TOML file path ($1)
    # - section ($2)
    # - the key ($3)
    # 
    # It first gets the section using the get_section function
    # Then it finds the key within that section
    # using grep and cut.

    local file="$1"
    local section="$2"
    local key="$3"

    get_section() {
        # Function to get the section from a TOML file
        # Takes two parameters:
        # - TOML file path ($1)
        # - section name ($2)
        # 
        # It uses sed to find the section
        # A section is terminated by a line with [ in pos 0 or the end of file.

        local file="$1"
        local section="$2"

        sed -n "/^\[$section\]/,/^\[/p" "$file" | sed '$d'
    }
        
    get_section "$file" "$section" | grep "^$key " | cut -d "=" -f2- | tr -d ' "'
}

# Set up our variables
app_name=$(get_toml_value "Cargo.toml" "package" "name")
app_version=$(get_toml_value "Cargo.toml" "package" "version")

# Build the UI
cd ui
npm install
npm run build
cd ..

for app_architecture in amd64 arm64
do
    if [ "$app_architecture" = "amd64" ]; then
        rust_target="x86_64-unknown-linux-gnu"
    elif [ "$app_architecture" = "arm64" ]; then
        rust_target="aarch64-unknown-linux-gnu"
        export PKG_CONFIG_SYSROOT_DIR="/usr/aarch64-linux-gnu"
    fi
    
    # Set up directories
    build_dir="target/${app_name}_$app_version-$app_architecture"
    rm -rf $build_dir
    mkdir -p $build_dir
    mkdir -p $build_dir/DEBIAN
    mkdir -p $build_dir/usr/local/bin
    mkdir -p $build_dir/lib/systemd/system
    mkdir -p $build_dir/etc/$app_name

    # Build the executable
    cargo build --release --target $rust_target
    cp target/$rust_target/release/$app_name $build_dir/usr/local/bin/
    chmod 755 $build_dir/usr/local/bin/$app_name

    # Add the package's config files
    cp config/control $build_dir/DEBIAN/
    cp config/conffiles $build_dir/DEBIAN/
    sed -i "s/VERSION/$app_version/" $build_dir/DEBIAN/control
    sed -i "s/ARCHITECTURE/$app_architecture/" $build_dir/DEBIAN/control
    cp config/postinst.sh $build_dir/DEBIAN/postinst
    chmod 755 $build_dir/DEBIAN/postinst
    cp config/prerm.sh $build_dir/DEBIAN/prerm
    chmod 755 $build_dir/DEBIAN/prerm
    cp config/yell-o.service $build_dir/lib/systemd/system/
    cp config/yell-o.env $build_dir/etc/$app_name/
    cp config/pulseaudio.service $build_dir/lib/systemd/system/

    # Add the UI
    cp -r ui/dist $build_dir/etc/$app_name/ui

    # Actually build the package
    dpkg-deb --build $build_dir

    # Clean up
    rm -rf $build_dir
done

echo "Done!"