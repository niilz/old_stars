import styles from './Logo.module.css';

interface LogoProps {
  styles: string;
}
export function AppLogo(props: LogoProps) {
  return <div className={`${styles.AppLogo} ${props.styles}`}>⭐</div>;
}
