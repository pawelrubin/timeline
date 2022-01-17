import 'package:hive/hive.dart';
import 'package:intl/intl.dart';

part 'location_entry.g.dart';

@HiveType(typeId: 0)
class LocationEntry extends HiveObject {
  LocationEntry(
      {required this.longitude,
      required this.latitude,
      required this.timestamp,
      required this.accuracy,
      required this.activity});

  @HiveField(0)
  double latitude;

  @HiveField(1)
  double longitude;

  @HiveField(2)
  DateTime timestamp;

  @HiveField(3)
  String activity;

  @HiveField(4)
  double accuracy;

  factory LocationEntry.fromJson(Map<String, dynamic> json) {
    return LocationEntry(
        longitude: json['lng'],
        latitude: json['lat'],
        timestamp: DateTime.parse(json['timestamp']),
        accuracy: json['accuracy'],
        activity: json['activity']);
  }

  Map<String, dynamic> toJson() {
    return {
      'lng': longitude,
      'lat': latitude,
      'timestamp': DateFormat('yyyy-MM-ddTH:m:s').format(timestamp),
      'accuracy': accuracy,
      'activity': activity
    };
  }
}
