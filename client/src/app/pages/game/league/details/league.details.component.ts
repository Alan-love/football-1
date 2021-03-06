import { Component, OnInit, ChangeDetectorRef } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { FootballApi, LeagueGetLeagueDto } from 'src/client/football.api.client';
import { HeaderService } from '../../services/header.service';

@Component({
  templateUrl: './league.details.component.html',
  styleUrls: ['./league.details.conponent.less']
})
export class LeagueDetailsComponent implements OnInit {
  isLoading: Boolean = false;

  league: LeagueGetLeagueDto;

  constructor(private api: FootballApi,
    private headerService: HeaderService, 
    private route: ActivatedRoute) {
  }

  ngOnInit() {
    this.route.parent.parent.params.subscribe(params => {
      
      this.isLoading = true;
      this.api.league(params["gameId"], this.route.snapshot.params.leagueId)
      .subscribe(data => {
        this.headerService.setHeader(data.league.name, 'League table');


        this.league = data.league;
        this.isLoading = false;
      })
    });
  }
}
