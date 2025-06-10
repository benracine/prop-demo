#!/usr/bin/env python3

import asyncio
import sys


async def read_stdin_and_send(writer):
    """Read from stdin and send to TCP connection"""
    loop = asyncio.get_event_loop()
    reader = asyncio.StreamReader()
    protocol = asyncio.StreamReaderProtocol(reader)
    await loop.connect_read_pipe(lambda: protocol, sys.stdin)

    try:
        while True:
            line = await reader.readline()
            if not line:
                break
            writer.write(line)
            await writer.drain()
    except (ConnectionResetError, asyncio.CancelledError):
        pass
    finally:
        writer.close()
        await writer.wait_closed()


async def read_tcp_and_print(reader):
    """Read from TCP connection and print decoded strings to stdout"""
    try:
        while True:
            data = await reader.read(1024)
            if not data:
                break
            text = data.decode('utf-8', errors='replace')
            print(text, end='', flush=True)
    except (ConnectionResetError, asyncio.CancelledError):
        pass


async def tcp_translation_client(command_port, log_port):
    """Main client function"""
    try:
        # Connect to the command port
        command_reader, command_writer = await asyncio.open_connection("127.0.0.1", command_port)
        print(f"\nReady to command over port {command_port}", file=sys.stderr)

        # Connect to the log port
        log_reader, _ = await asyncio.open_connection("127.0.0.1", log_port)
        print(f"Ready for telemetry over port {log_port}\n", file=sys.stderr)

        # Start tasks for reading logs and sending commands
        stdin_task = asyncio.create_task(read_stdin_and_send(command_writer))
        log_task = asyncio.create_task(read_tcp_and_print(log_reader))

        await asyncio.wait([stdin_task, log_task], return_when=asyncio.FIRST_COMPLETED)

        # Clean up remaining tasks
        stdin_task.cancel()
        log_task.cancel()
        await asyncio.gather(stdin_task, log_task, return_exceptions=True)

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
    finally:
        print("\nDisconnected", file=sys.stderr)


def main():
    command_port = 8124
    log_port = 8125

    try:
        asyncio.run(tcp_translation_client(command_port, log_port))
    except KeyboardInterrupt:
        print("\nExiting...", file=sys.stderr)


if __name__ == "__main__":
    main()
