import { useEffect, useState, useCallback } from 'react';
import { wsClient, WebSocketMessage } from '../api/websocket';

/**
 * React hook for WebSocket connection
 */
export function useWebSocket() {
  const [isConnected, setIsConnected] = useState(false);
  const [lastMessage, setLastMessage] = useState<WebSocketMessage | null>(null);
  const [error, setError] = useState<Event | null>(null);

  useEffect(() => {
    // Connect on mount
    wsClient.connect();

    // Setup handlers
    const unsubscribeConnect = wsClient.onConnect(() => {
      setIsConnected(true);
      setError(null);
    });

    const unsubscribeDisconnect = wsClient.onDisconnect(() => {
      setIsConnected(false);
    });

    const unsubscribeMessage = wsClient.onMessage((message) => {
      setLastMessage(message);
    });

    const unsubscribeError = wsClient.onError((err) => {
      setError(err);
    });

    // Cleanup on unmount
    return () => {
      unsubscribeConnect();
      unsubscribeDisconnect();
      unsubscribeMessage();
      unsubscribeError();
      wsClient.disconnect();
    };
  }, []);

  const send = useCallback((data: unknown) => {
    wsClient.send(data);
  }, []);

  return {
    isConnected,
    lastMessage,
    error,
    send,
  };
}

/**
 * React hook for subscribing to specific WebSocket message types
 */
export function useWebSocketMessage(
  messageType: string,
  handler: (data: unknown) => void
) {
  const { lastMessage } = useWebSocket();

  useEffect(() => {
    if (lastMessage && lastMessage.type === messageType) {
      handler(lastMessage.data);
    }
  }, [lastMessage, messageType, handler]);
}
