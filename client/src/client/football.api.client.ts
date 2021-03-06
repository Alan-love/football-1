import { Injectable } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Observable, Subject } from "rxjs";

@Injectable({
  providedIn: 'root',
})
export class FootballApi {
  constructor(private http: HttpClient) {
  }

  games(): Observable<GameListResultDto> {
    const result = new Subject<GameListResultDto>();

    this.http.get('/api/game').subscribe((data: GameListResultDto) => {
      result.next(data);
    });

    return result;
  }

  countries(gameId): Observable<CountryListResponse> {
    const result = new Subject<CountryListResponse>();

    this.http.get('/api/game/' + gameId + '/countries').subscribe((data: CountryListResponse) => {
      result.next(data);
    });

    return result;
  }

  country(gameId, countryId): Observable<CountryGetResponse> {
    const result = new Subject<CountryGetResponse>();

    this.http.get('/api/game/' + gameId + '/countries/' + countryId).subscribe((data: CountryGetResponse) => {
      result.next(data);
    });

    return result;
  }

  league(gameId, leagueId): Observable<LeagueGetResponse> {
    const result = new Subject<LeagueGetResponse>();

    this.http.get('/api/game/' + gameId + '/league/' + leagueId).subscribe((data: LeagueGetResponse) => {
      result.next(data);
    });

    return result;
  }

  createGame() {
    const result = new Subject<GameCreateResultDto>();

    this.http.post('/api/game/create', {}).subscribe((data: GameCreateResultDto) => {
      result.next(data);
    });

    return result;
  }

  processGame(gameId) {
    const result = new Subject<GameProcessResultDto>();

    this.http.post('/api/game/' + gameId + '/process', {}).subscribe((data: GameProcessResultDto) => {
      result.next(data);
    });

    return result;
  }
}

export class GameCreateResultDto {
  game_id: String;
  elapsed: Number;
}

export class GameProcessResultDto {
  game_id: String;
  elapsed: Number;
}

export class GameListResultDto {
  games: GameListDto[]
}

export class GameListDto {
  id: String;
  date: String;
}

// Countries

export class CountryListResponse {
  continents: ContinentDto[]
}

export class ContinentDto {
  name: String;
  countries: CountryDto[];
}


// Countries

export class CountryDto {
  id: String;
  code: String;
  name: String;
  leagues: LeagueDto[];
}

export class LeagueDto {
  id: String;
  name: String;
}

export class CountryGetResponse {
  country: CountryGetCountryDto
}

export class CountryGetCountryDto {
  id: String;
  name: String;
  leagues: CountryGetLeagueDto[];
}

export class CountryGetLeagueDto {
  id: String;
  name: String;
}

// league

export class LeagueGetResponse {
  league: LeagueGetLeagueDto
}

export class LeagueGetLeagueDto {
  id: Number;
  name: String;
  table: LeagueGetLeagueTableDto;
  week_schedule: LeagueScheduleDto
}

export class LeagueGetLeagueTableDto {
  rows: LeagueGetLeagueTableRowDto[];
}

export class LeagueGetLeagueTableRowDto {
  club_id: Number;
  club_name: String;
  played: Number;
  win: Number;
  draft: Number;
  lost: Number;
  goal_scored: Number;
  goal_concerned: Number;
  points: Number;
}

export class LeagueScheduleDto {
  items: LeagueScheduleItemDto[];
}

export class LeagueScheduleItemDto {
  home_goals: Number;
  away_goals: Number;
    
  home_club_id: Number;
  home_club_name: String;
    
  away_club_id: Number;
  away_club_name: String;
}