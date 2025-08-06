import './index.css'
export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <head>
        <title>Old Stars</title>
        <meta
          name="description"
          content="Web app to accompany the yearly meeting"
        />
      </head>
      <body>
        <div id="root">{children}</div>
      </body>
    </html>
  )
}
