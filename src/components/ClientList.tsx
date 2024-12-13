import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import styles from './ClientList.module.scss';

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
      <h3>Connected Clients ({clients.length})</h3>
      {clients.length === 0 ? (
        <p className={styles.noClients}>No clients connected</p>
      ) : (
        <div className={styles.clients}>
          {clients.map((client) => (
            <div key={client.id} className={styles.client}>
              <div className={styles.header}>
                <div className={styles.name}>
                  {client.clientInfo.name} v{client.clientInfo.version}
                </div>
                <div className={styles.id}>{client.id}</div>
              </div>
              <div className={styles.details}>
                <div className={styles.time}>
                  Connected: {new Date(client.connectedAt).toLocaleString()}
                </div>
                <div className={styles.capabilities}>
                  <div>Experimental: {Object.keys(client.capabilities.experimental).length} features</div>
                  <div>Roots: {client.capabilities.roots.list_changed ? 'Enabled' : 'Disabled'}</div>
                  <div>Sampling: {Object.keys(client.capabilities.sampling).length} options</div>
                </div>
                <div className={styles.lastMessage}>
                  Last message: {client.lastMessage}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default ClientList;