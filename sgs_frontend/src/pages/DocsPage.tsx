import { Resources } from '../components/Resources';
import '../App.css';

export function DocsPage() {
  return (
    <>
      <section className="docs-hero">
        <div className="hero-content">
          <div className="hero-actions">
            <a
              className="button primary"
              href="https://github.com/jamesbachini/Stellar-Game-Studio"
              target="_blank"
              rel="noreferrer"
            >
              Fork the repo
            </a>
            <a className="button ghost" href="#quickstart">
              Read the quickstart
            </a>
          </div>
          <div className="hero-tags">
            <span>TEMPLATES &AMP; EXAMPLES</span>
            <span>ECOSYSTEM COMPATIBLE</span>
            <span>ZK READY</span>
          </div>
        </div>

        <div className="hero-panel">
          <div className="hero-panel-header">Dev-to-Publish Pipeline</div>
          <ol className="hero-steps">
            <li>Fork and clone the repo</li>
            <li>Deploy contracts to testnet</li>
            <li>Build the standalone game frontend</li>
            <li>Publish with a production wallet flow</li>
          </ol>
          <div className="hero-code">
            <pre>
              <code>{`bun run setup
bun run create my-game
bun run dev:game my-game
bun run publish my-game --build`}</code>
            </pre>
          </div>
        </div>
      </section>

      <Resources />
    </>
  );
}
