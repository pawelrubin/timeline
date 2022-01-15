import 'dart:convert';

import 'package:firebase_auth/firebase_auth.dart';
import 'package:timeline/modules/core/model/location_entry.dart';
import 'package:http/http.dart' as http;

class ApiService {
  String apiBaseUrl;

  ApiService({required this.apiBaseUrl});

  Future<void> updateData(List<LocationEntry> data) async {
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

  Future<List<LocationEntry>> fetchData() async {
    var token = await FirebaseAuth.instance.currentUser!.getIdToken();
    var url = Uri.parse('$apiBaseUrl/geodata');
    var headers = {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer $token'
    };

    http.Response response = await http.get(url, headers: headers);
    List<dynamic> data = jsonDecode(response.body);

    return data.map((e) => LocationEntry.fromJson(e)).toList();
  }
}
