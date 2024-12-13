import * as React from "react"
import styles from "./Card.module.scss"

interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  children?: React.ReactNode;
  title?: string | React.ReactNode;
  mode?: 'left' | 'right';
}

const Card: React.FC<CardProps> = ({ children, mode, title, ...rest }) => {
  return (
    <article className={styles.card} {...rest}>
      {title && (
        <header className={styles.action}>
          {mode === 'left' && <div className={styles.leftCorner} aria-hidden="true" />}
          {!mode && <div className={styles.left} aria-hidden="true" />}
          <h2 className={styles.title}>{title}</h2>
          {!mode && <div className={styles.right} aria-hidden="true" />}
          {mode === 'right' && <div className={styles.rightCorner} aria-hidden="true" />}
        </header>
      )}
      <section className={styles.children}>{children}</section>
    </article>
  );
};

export default Card;