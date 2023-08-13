import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, map } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class BackendService {

  private _rpcUrl = 'http://localhost:8080';

  constructor(private _http: HttpClient) { }

  private makeRpcCall<T>(type: string, data: any): Observable<T> {
    const headers = new HttpHeaders().set('Content-Type', 'application/json');
    const payload = {
      [type]: data
    };

    return this._http.post<T>(this._rpcUrl, payload, { headers });
  }
  public test(a: number, b: number): Observable<{ 'res': number }> {
    const payload = {
      'a': a,
      'b': b
    };

    const res: Observable<{'ExampleResult': { 'res': number }}> = this.makeRpcCall('Example', payload);
    return res.pipe(map(item => item.ExampleResult));
  }
}
