import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  output: 'export', // Outputs a Single-Page Application (SPA)
  distDir: 'build', // Changes the build output directory to `build`
  basePath: '/old_stars', // gh_pages requires to define a basePath
}

export default nextConfig
