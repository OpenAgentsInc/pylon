import React, { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core"
import styles from "./ClientList.module.scss"
import Card from "./Card"

interface Client {
  id: string;
  clientInfo: {
    name: string;
    version: string;
  };
  connectedAt: string;
  lastMessage: string;
  capabilities: {
    experimental: Record<string, any>;
    roots: {
      list_changed: boolean;
    };
    sampling: Record<string, any>;
  };
}

const ClientList: React.FC = () => {
  const [clients, setClients] = useState<Client[]>([]);

  useEffect(() => {
    const fetchClients = async () => {
      try {
        const connectedClients = await invoke<Client[]>('get_connected_clients');
        console.log('Connected clients:', connectedClients);
        setClients(connectedClients);
      } catch (error) {
        console.error('Error fetching clients:', error);
      }
    };

    // Initial fetch
    fetchClients();

    // Set up interval to fetch every 2 seconds
    const interval = setInterval(fetchClients, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className={styles.clientList}>
      <h3 className={styles.title}>Connected Clients ({clients.length})</h3>
      {clients.length === 0 ? (
        <Card>
          <p className={styles.noClients}>No clients connected</p>
        </Card>
      ) : (
        <div className={styles.grid}>
          {clients.map((client) => (
            <Card key={client.id}>
              <div className={styles.client}>
                <div className={styles.header}>
                  <div className={styles.name}>
                    {client.clientInfo.name} <span className={styles.version}>v{client.clientInfo.version}</span>
                  </div>
                  <div className={styles.id}>{client.id}</div>
                </div>
                <div className={styles.details}>
                  <div className={styles.time}>
                    Connected: {new Date(client.connectedAt).toLocaleString()}
                  </div>
                  <div className={styles.capabilities}>
                    <div className={styles.capability}>
                      <span className={styles.label}>Experimental:</span>
                      <span className={styles.value}>{Object.keys(client.capabilities.experimental).length} features</span>
                    </div>
                    <div className={styles.capability}>
                      <span className={styles.label}>Roots:</span>
                      <span className={styles.value}>{client.capabilities.roots.list_changed ? 'Enabled' : 'Disabled'}</span>
                    </div>
                    <div className={styles.capability}>
                      <span className={styles.label}>Sampling:</span>
                      <span className={styles.value}>{Object.keys(client.capabilities.sampling).length} options</span>
                    </div>
                  </div>
                  <div className={styles.lastMessage}>
                    <span className={styles.label}>Last message:</span>
                    <span className={styles.value}>{client.lastMessage}</span>
                  </div>
                </div>
              </div>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
};

export default ClientList;