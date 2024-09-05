import { Game, DateRecord } from './game.model'

export interface GameResponse {
  status: string,
  data: Game,
}

export interface RecordResponse {
  status: string,
  results: number,
  dates: DateRecord[],
}

export interface ErrorResponse {
  status: string,
  message: string,
}
