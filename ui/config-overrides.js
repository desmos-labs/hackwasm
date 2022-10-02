const webpack = require('webpack');
module.exports = function override(config, env) {
    config.resolve.fallback = {
        buffer: false,
        crypto: false,
        events: false,
        path: false,
        stream: false,
        string_decoder: false,
    };
    config.plugins.push(
        new webpack.ProvidePlugin({
            Buffer: ['buffer', 'Buffer'],
        }),
    );

    return config;
}
