import 'package:timeline/modules/core/model/location_entry.dart';
import 'package:http/http.dart' as http;
import 'package:timeline/modules/core/services/database_service.dart';

class ApiService {
  String apiBaseUrl;
  DatabaseService? database;

  ApiService({required this.apiBaseUrl, required this.database});

  Future<void> updateData(List<LocationEntry> data) async {
    var url = Uri.parse('$apiBaseUrl/update');
    await http.post(url, body: data);
  }

  fetchData() async {
    var url = Uri.parse('$apiBaseUrl/data');
    return await http.get(url);
  }
}
