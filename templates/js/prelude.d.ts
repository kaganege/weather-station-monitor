// Refer to src/network/web/handler/routes/api.rs
declare interface Data {
    windDirection: number | null;
    windSpeed1Min: number | null;
    maxWindSpeed5Min: number | null;
    temperature: number | null;
    rainfall1Hour: number | null;
    rainfall1Day: number | null;
    humidity: number | null;
    airPressure: number | null;
}

declare const ENDPOINT: string;
