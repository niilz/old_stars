import './Logo.css';

interface LogoProps {
  addClass: string;
}

export function AppLogo(props: LogoProps) {
  return <div className={`App-Logo ${props.addClass}`}>⭐</div>;
}
