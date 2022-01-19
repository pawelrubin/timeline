import 'dart:convert';

import 'package:firebase_auth/firebase_auth.dart';
import 'package:timeline/modules/core/model/location_entry.dart';
import 'package:http/http.dart' as http;

const CHUNK_SIZE = 100;

class ApiService {
  String apiBaseUrl;

  ApiService({required this.apiBaseUrl});

  Future<void> _updateBatchData(List<LocationEntry> data) async {
    var token = await FirebaseAuth.instance.currentUser!.getIdToken();
    var url = Uri.parse('$apiBaseUrl/geodata');
    var headers = {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer $token'
    };
    var body = jsonEncode(data);

    await http.post(
      url,
      headers: headers,
      body: body,
    );
  }

  Future<void> updateData(List<LocationEntry> data) async {
    for (var i = 0; i < data.length; i += CHUNK_SIZE) {
      var batch = data.sublist(
          i, i + CHUNK_SIZE > data.length ? data.length : i + CHUNK_SIZE);
      await _updateBatchData(batch);
    }
  }

  Future<List<LocationEntry>> fetchData(DateTime date) async {
    var token = await FirebaseAuth.instance.currentUser!.getIdToken();
    var url = Uri.parse(
        '$apiBaseUrl/geodata?on_day=${date.year}-${date.month}-${date.day}');
    var headers = {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer $token'
    };

    print(url);

    http.Response response = await http.get(url, headers: headers);
    List<dynamic> data = jsonDecode(response.body);

    return data.map((e) => LocationEntry.fromJson(e)).toList();
  }
}
