import './Header.css';

interface HeaderProps {
  text: string;
}
export function Header(props: HeaderProps) {
  return (
    <header className="App-header">
      <h1 className="title">{props.text}</h1>
    </header>
  );
}
