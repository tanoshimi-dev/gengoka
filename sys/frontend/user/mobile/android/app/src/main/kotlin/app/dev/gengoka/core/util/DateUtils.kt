package app.dev.gengoka.core.util

import java.time.Instant
import java.time.LocalDateTime
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import java.time.temporal.ChronoUnit

object DateUtils {

    fun formatRelativeTime(isoDateTime: String): String {
        return try {
            val instant = Instant.parse(isoDateTime)
            val now = Instant.now()
            val minutes = ChronoUnit.MINUTES.between(instant, now)
            val hours = ChronoUnit.HOURS.between(instant, now)
            val days = ChronoUnit.DAYS.between(instant, now)

            when {
                minutes < 1 -> "たった今"
                minutes < 60 -> "${minutes}分前"
                hours < 24 -> "${hours}時間前"
                days < 7 -> "${days}日前"
                else -> {
                    val localDateTime = LocalDateTime.ofInstant(instant, ZoneId.systemDefault())
                    localDateTime.format(DateTimeFormatter.ofPattern("M/d"))
                }
            }
        } catch (e: Exception) {
            isoDateTime
        }
    }

    fun formatDateTime(isoDateTime: String): String {
        return try {
            val instant = Instant.parse(isoDateTime)
            val localDateTime = LocalDateTime.ofInstant(instant, ZoneId.systemDefault())
            localDateTime.format(DateTimeFormatter.ofPattern("yyyy/M/d HH:mm"))
        } catch (e: Exception) {
            isoDateTime
        }
    }
}
