import { Idl, Event, EventCoder } from "@solarti/anchor";
import { IdlEvent } from "@solarti/anchor/dist/cjs/idl";

export class SplTokenSwapEventsCoder implements EventCoder {
  constructor(_idl: Idl) {}

  decode<E extends IdlEvent = IdlEvent, T = Record<string, string>>(
    _log: string
  ): Event<E, T> | null {
    throw new Error("SplTokenSwap program does not have events");
  }
}
