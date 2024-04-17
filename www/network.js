export async function downloadFile(url, progressCallback)
{
    if (!url) {
        alert('No URL provided');
        return;
    }
    const response = await fetch(url);
    const contentLength = parseInt(
        response.headers.get('content-length'),
        10);  // Parse content-length as integer
    const reader = response.body.getReader();
    let receivedLength = 0;  // Track the total number of bytes downloaded
    let chunks = [];         // Array to store downloaded chunks

    while (true) {
        const {done, value} = await reader.read();

        if (done) {
            break;
        }

        chunks.push(value);
        receivedLength += value.length;
        progressCallback(receivedLength, contentLength);
    }

    // Concatenate all the downloaded chunks into a single Uint8Array
    const totalLength = chunks.reduce((acc, chunk) => acc + chunk.length, 0);
    let offset = 0;
    const uint8Array = new Uint8Array(totalLength);

    for (const chunk of chunks) {
        uint8Array.set(chunk, offset);
        offset += chunk.length;
    }

    return uint8Array;
}
