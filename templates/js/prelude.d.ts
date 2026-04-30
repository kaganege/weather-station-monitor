// Refer to src/network/web/handler/routes/api.rs

declare type WindDirection = "north" | "north-east" | "east" | "south" | "south-east" | "south-west" | "west" | "north-west";

declare interface Data {
    windDirection: WindDirection | null;
    windSpeed1Min: number | null;
    maxWindSpeed5Min: number | null;
    temperature: number | null;
    rainfall1Hour: number | null;
    rainfall1Day: number | null;
    humidity: number | null;
    airPressure: number | null;
}
