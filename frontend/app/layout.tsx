import './index.css'

import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'Old Stars',
  description: 'Web app to accompany the yearly meeting',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <div id="root">{children}</div>
      </body>
    </html>
  )
}
