export default function getUrl(): string {
    if (import.meta.env.MODE === "production") {
        // In production, make sure the URL uses the same port as the page we're on, if any
        return `${window.location.protocol}//${window.location.host}`;
    } else {
        // Assume we want to use the Flask dev server as well
        return `${window.location.protocol}//${window.location.hostname}:5000`;
    }
}