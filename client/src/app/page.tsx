import { validateUser } from "@/actions/auth";
import styles from "@/app/page.module.css";

export default async function Home() {
  const user = await validateUser();

  return (
    <main className={styles.main}>
      {user && <p>Hello {user.username}!</p>}
      {!user && <p>I don&apos;t know you!</p>}
    </main>
  );
}
