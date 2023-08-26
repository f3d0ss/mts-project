// @ts-check

/** @type {import('next').NextConfig} */
const nextConfig = {
  // reactStrictMode is false to avoid qr reader problem: https://github.com/scanapp-org/html5-qrcode-react/issues/9
  reactStrictMode: false,
  typescript: {
    ignoreBuildErrors: process.env.NEXT_PUBLIC_IGNORE_BUILD_ERROR === "true",
  },
  eslint: {
    ignoreDuringBuilds: process.env.NEXT_PUBLIC_IGNORE_BUILD_ERROR === "true",
  },
};

module.exports = nextConfig;
